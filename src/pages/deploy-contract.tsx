import { useEffect, useState } from 'react'
import { useStatus } from '../hooks'
import { invoke } from '@tauri-apps/api/tauri'
import { ethers } from 'ethers'
import testKeystore from '../test_keystore.json'
import { toast } from 'react-hot-toast'
import { dialog } from '@tauri-apps/api'
import { CreateAddressesPage } from './create_addresses'

type ContractType = 'vyper' | 'stylus' | 'solidity'

export const DeployContractPage = () => {
  const [config, setConfig] = useState<{
    provider: string
    etherscan_api: string
  }>()
  const [keys, setKeys] = useState<
    {
      name: string
      path: string
    }[]
  >([])

  const [contractName, setContractName] = useState('')
  const [pathToContract, setPathToContract] = useState('')
  const [evmVersion, setEvmVersion] = useState('Cancun')
  const [keyToUse, setKeyToUse] = useState<string>('')
  const [constructorArgs, setConstructorArgs] = useState<string>('')
  const [contractType, setContractType] = useState<ContractType>('vyper')
  const [gasEstimate, setGasEstimate] = useState<string>('')
  const [password, setPassword] = useState<string>('')
  const [wallet, setWallet] = useState<ethers.Wallet>()

  const hasConfig = config?.etherscan_api && config?.provider
  const args = localStorage.getItem('args')?.split(',')

  const [status, setStatus] = useStatus('idle')
  const provider = new ethers.JsonRpcProvider(config?.provider)

  useEffect(() => {
    ;(async () => {
      const config = (await invoke('get_config')) as {
        provider: string
        etherscan_api: string
      }
      setConfig(config)
    })()
  }, [])

  useEffect(() => {
    ;(async () => {
      const keys = await invoke('list_keys').catch(e => console.log(e))
      setKeys(
        keys as {
          name: string
          path: string
        }[]
      )
    })()
  }, [])

  useEffect(() => {
    const makeWallet = async (pass: String) => {
      const key: any = await invoke('get_key_by_name', {
        name: keyToUse,
        password: pass
      })
      setWallet(new ethers.Wallet(`0x${key.pk}`).connect(provider))
    }
    makeWallet(password)
  }, [keyToUse, password])

  useEffect(() => {
    setContractName(
      pathToContract
        .split('/')
        [pathToContract.split('/').length - 1].split('.')[0]
    )
    if (contractName.length === pathToContract.length) {
      setContractName(
        pathToContract
          .split('/\\/')
          [pathToContract.split('/\\/').length - 1].split('.')[0]
      )
    }
  }, [pathToContract])

  const compileVyperContract = async () => {
    const res: {
      abi: Array<any>
      initcode: string
    } = await invoke('compile_version', {
      path: pathToContract,
      version: evmVersion
    })
    const abi = new ethers.Interface(res.abi)
    const initcode = res.initcode
    return { abi, initcode }
  }

  const deployVyperContract = async () => {
    const { abi, initcode } = await compileVyperContract()
    console.log(abi, initcode)
    const contractFactory = new ethers.ContractFactory(
      abi,
      {
        object: initcode
      },
      wallet
    )
    console.log(contractFactory)
    console.log(args)

    const tx = await contractFactory.deploy(args)
    await tx.waitForDeployment()
    const contractAddress = await tx.getAddress()

    await writeDeploymentToDb({
      deploymentAddress: contractAddress,
      deployerAddress: wallet.address,
      smartContractName: contractName,
      chainId: (await provider.getNetwork()).chainId.toString()
    })
  }

  //keystore path is tracked in state

  const deployStylusContract = async () => {
    const key: any = await invoke('get_key_by_name', {
      name: keyToUse,
      password: password
    })
    const {
      fee,
      deployment_address
    }: {
      fee: string
      deployment_address: string
    } = await invoke('stylus_deploy_contract', {
      rootPath: pathToContract,
      keystorePath: key.path,
      pass: password
    })
    await writeDeploymentToDb({
      deploymentAddress: deployment_address,
      //double check me :) <3 bitch
      deployerAddress: key?.address,
      smartContractName: contractName,
      chainId: wallet?.provider?.getNetwork.name,
      fee
    })
  }

  const deploySolidityContract = async () => {
    const res = await invoke('compile_solidity', {
      filePath: pathToContract,
      outputPath: '../'
    })
    const json = JSON.parse(res)
    const abi = new ethers.Interface(JSON.parse(json.abi))
    const initcode = json.bytecode

    const provider = new ethers.JsonRpcProvider(config?.provider)
    const wallet = (
      await ethers.Wallet.fromEncryptedJson(
        JSON.stringify(testKeystore),
        'abcd'
      )
    ).connect(provider)

    const contractFactory = new ethers.ContractFactory(
      abi,
      {
        object: initcode
      },
      wallet
    )

    console.log(constructorArgs)

    const tx = await contractFactory.deploy(args)
    await tx.waitForDeployment()
    const contractAddress = await tx.getAddress()

    await writeDeploymentToDb({
      deploymentAddress: contractAddress,
      deployerAddress: wallet.address,
      smartContractName: contractName,
      chainId: (await provider.getNetwork()).chainId.toString()
    })
  }

  const onSubmit = async () => {
    try {
      setStatus('loading')
      if (contractType === 'vyper') {
        await deployVyperContract()
      }

      if (contractType === 'stylus') {
        await deployStylusContract()
      }

      if (contractType === 'solidity') {
        await deploySolidityContract()
      }
    } catch (error) {
      console.log(error)
      setStatus('error')
      toast.error('Error deploying contract')
    } finally {
      setStatus('success')
      toast.success('Contract deployed')
    }
  }

  const writeDeploymentToDb = async ({
    deploymentAddress,
    deployerAddress,
    smartContractName,
    chainId,
    fee
  }: {
    deploymentAddress: string
    deployerAddress: string
    smartContractName: string
    chainId: string
    fee?: string
  }) => {
    const res = await invoke('db_write', {
      deploymentData: {
        sc_name: smartContractName,
        deployer_address: deployerAddress,
        deploy_date: new Date().toISOString(),
        sc_address: deploymentAddress,
        network: chainId,
        fee: fee || '0',
        verified: false
      }
    })
    console.log(res)
  }

  const estimateVyperGas = async () => {
    const { abi, initcode } = await compileVyperContract()
    const provider = new ethers.JsonRpcProvider(config?.provider)
    const wallet = (
      await ethers.Wallet.fromEncryptedJson(
        JSON.stringify(testKeystore),
        'abcd'
      )
    ).connect(provider)

    const contractFactory = new ethers.ContractFactory(
      abi,
      {
        object: initcode
      },
      wallet
    )

    const tx = await contractFactory.getDeployTransaction(1, [
      '0x0ED6Cec17F860fb54E21D154b49DAEFd9Ca04106'
    ])

    const gas = await wallet.estimateGas(tx)

    setGasEstimate(gas.toString())
  }

  const estimateStylusGas = async () => {
    const res = await invoke('stylus_estimate_gas', {
      rootPath: pathToContract,
      keystorePath: './test_keystore.json',
      pass: 'abcd'
    })

    setGasEstimate(res!.toString())
  }

  const handleEstimateGas = async () => {
    try {
      setStatus('loading')
      if (contractType === 'vyper') {
        await estimateVyperGas()
      }

      if (contractType === 'stylus') {
        await estimateStylusGas()
      }
    } catch (error) {
      setStatus('error')
      toast.error('Error estimating gas')
    } finally {
      setStatus('success')
      toast.success('Gas estimated')
    }
  }

  if (!hasConfig) {
    return <div>No config found</div>
  }

  return (
    <div className='flex flex-col gap-4'>
      <div className='form-control'>
        <label htmlFor='keys' className='label'>
          Key To Use
        </label>
        <select
          className='select select-bordered'
          id='keys'
          value={keyToUse}
          onChange={e => setKeyToUse(e.target.value)}
        >
          {keys.length ? (
            keys.map(key => (
              <option key={key.name} value={key.name}>
                {key.name}
              </option>
            ))
          ) : (
            <option key={1} value={''}>
              {'No Valid Keys'}
            </option>
          )}
        </select>
      </div>

      <div className='form-control'>
        <label htmlFor='password' className='label'>
          Input Wallet Password
        </label>
        <input
          type='password'
          className='input input-bordered'
          id='password'
          value={password}
          onChange={e => setPassword(e.target.value)}
        />
      </div>

      <div>
        {wallet
          ? `Address: ${wallet.address}`
          : 'please fill form to see address'}
      </div>

      <CreateAddressesPage />

      <div className='form-control'>
        <label htmlFor='contractType' className='label'>
          Contract Type
        </label>
        <select
          className='select select-bordered'
          id='contractType'
          value={contractType}
          onChange={e => setContractType(e.target.value as ContractType)}
        >
          <option value='vyper'>Vyper 🐍</option>
          <option value='stylus'>Stylus 🖋️</option>
          <option value='solidity'>Solidity 🧱</option>
        </select>
      </div>

      <div className='form-control'>
        <label htmlFor='pathToContract' className='label'>
          Path To Contract
        </label>
        <div>
          <button
            className='btn btn-outline'
            id='pathToContract'
            value={pathToContract}
            onClick={e => {
              e.preventDefault()
              ;(async () => {
                const temp = await dialog.open({
                  filters: [
                    { name: 'vyper', extensions: ['vy'] },
                    { name: 'solidity', extensions: ['sol'] },
                    { name: 'stylus', extensions: ['styl'] }
                  ]
                })
                setPathToContract(temp)
              })()
            }}
          >
            Select File
          </button>
          <label className='pl-6'>{pathToContract}</label>
        </div>
      </div>

      <div className='form-control'>
        <label htmlFor='evmVersion' className='label'>
          EVM Version
        </label>
        <select
          className='select select-bordered'
          id='evmVersion'
          value={evmVersion}
          onChange={e => setEvmVersion(e.target.value)}
        >
          <option value='cancun'>Cancun</option>
          <option value='shanghai'>Shanghai</option>
          <option value='berlin'>Berlin</option>
          <option value='paris'>Paris</option>
          <option value='london'>London</option>
        </select>
      </div>

      {/* <div className='form-control'>
        <label htmlFor='constructorArgs' className='label'>
          Constructor Arguments
        </label>
        <input
          className='input input-bordered'
          type='text'
          id='constructorArgs'
          value={constructorArgs}
          onChange={e => setConstructorArgs(e.target.value)}
        />
      </div> */}
      <div>
        {gasEstimate
          ? `Gas Estimate: ${gasEstimate}`
          : 'Please complete form to see gas estimate'}
      </div>
      {/* 
      {gasEstimate && (
        <span className='text-sm'>Gas Estimate: {gasEstimate} wei</span>
      )} */}

      <button className='btn btn-outline' onClick={handleEstimateGas}>
        {status === 'loading' && (
          <span className='loading loading-spinner'></span>
        )}
        Estimate Gas
      </button>

      <button className='btn btn-primary' onClick={onSubmit}>
        {status === 'loading' && (
          <span className='loading loading-spinner'></span>
        )}
        Deploy Contract
      </button>
    </div>
  )
}
