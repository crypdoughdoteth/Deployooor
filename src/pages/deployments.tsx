import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from 'react'

type Deployment = {
  sc_name: string;
  deployer_address: string;
  deploy_date: string;
  sc_address: string;
  network: string;
  fee: string;
  verified: boolean;
}

const chainIds = {
  11155111: 'ETH Sepolia',
  23011913: 'Arbitrum Stylus Testnet',
}

export const DeploymentsPage = () => {
  const [deployments, setDeployments] = useState<Deployment[]>([]);

  useEffect(() => {
    (async () => {
      const deployments = await invoke('db_read') as Deployment[];
      setDeployments(deployments);
    })()
  }, [])

  return (
    <div className='flex flex-col gap-4'>
      <table className='table table-zebra'>
        <thead>
          <tr>
            <th>SC Name</th>
            <th>Deployer Address</th>
            <th>Deploy Date</th>
            <th>SC Address</th>
            <th>Network</th>
            <th>Fee</th>
            <th>Verified</th>
          </tr>
        </thead>

        <tbody>
          {deployments.map((deployment) => (
            <tr key={deployment.sc_name}>
              <td>{deployment.sc_name}</td>
              <td>{deployment.deployer_address}</td>
              <td>{deployment.deploy_date}</td>
              <td>{deployment.sc_address}</td>
              {/* @ts-ignore */}
              <td>{chainIds[deployment.network] as string}</td>
              <td>{deployment.fee}</td>
              <td>{deployment.verified ? 'Yes' : 'No'}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}