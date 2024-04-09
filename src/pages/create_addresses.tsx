import React, { useState } from 'react'

export const CreateAddressesPage = () => {
  const [args, setArgs] = useState<any[]>([])
  const [arg, setArg] = useState<any>('')

  return (
    <div className='flex flex-col gap-4'>
      <input
        type='text'
        onChange={e => setArg(e.target.value)}
        placeholder='Argument'
        className='input input-bordered'
      ></input>
      <button
        className='btn btn-outline'
        onClick={e => {
          e.preventDefault()
          setArgs([...args, arg])
          setArg('')
        }}
      >
        Add Constructor Arguments
      </button>
      <details className='dropdown cursor-pointer'>
        <summary>List All Arguments</summary>
        <ul>
          {args.map((i, idx) => (
            <li key={idx}>{i}</li>
          ))}
        </ul>
      </details>
    </div>
  )
}
