import React, { useEffect, useState } from 'react'

export const CreateAddressesPage = props => {
  const { args, setArgs } = props

  const [arg, setArg] = useState<any>('')
  const [type, setType] = useState<any>('')

  const onDeleteArg = (idx: number) => {
    setArgs(args.filter((_, i) => i !== idx))
  }

  // useEffect(() => {
  //   localStorage.setItem('args', args.toString())

  // }, [args])

  return (
    <div className='flex flex-col gap-4'>
      <input
        type='text'
        value={arg}
        onChange={e => setArg(e.target.value)}
        placeholder='Argument'
        className='input input-bordered'
      ></input>

      <select
        value={type}
        onChange={e => setType(e.target.value)}
        className='select select-bordered'
      >
        <option value=''>Select Type</option>
        <option value='Address'>Address</option>
        <option value='Int'>int</option>
        <option value='Uint'>Uint</option>
        <option value='Array'>Array</option>
        <option value='FixedArray'>Fixed Array</option>
        <option value='Bytes'>Bytes</option>
        <option value='FixedBytes'>Fixed Bytes</option>
        <option value='Struct'>Struct</option>
        <option value='Bool'>Bool</option>
      </select>

      <button
        className='btn btn-outline'
        onClick={e => {
          e.preventDefault()
          if (!arg) return alert('Please enter an argument')
          if (!type) return alert('Please select a type')
          else {
            setArgs([...args, { type: type, arg: arg }])
            setArg('')
            setType('')
          }
        }}
      >
        Add Constructor Arguments
      </button>
      <details className='dropdown cursor-pointer'>
        <summary>List All Arguments</summary>
        <ul>
          {args.map((i, idx) => (
            <li key={idx}>
              {i.arg} |{' '}
              <button
                className='text-red-900'
                onClick={() => {
                  setArgs(args.filter((_, i) => i !== idx))
                }}
              >
                delete
              </button>
            </li>
          ))}
        </ul>
      </details>
    </div>
  )
}
