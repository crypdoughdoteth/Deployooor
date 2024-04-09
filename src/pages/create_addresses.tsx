import React, { useEffect, useState } from 'react'

export const CreateAddressesPage = () => {
  const [args, setArgs] = useState<any[]>([])
  const [arg, setArg] = useState<any>('')

  useEffect(() => {
    localStorage.setItem('args', JSON.stringify(args))
  }, [args])

  return (
    <div className='flex flex-col gap-4'>
      <input
        type='text'
        value={arg}
        onChange={e => setArg(e.target.value)}
        placeholder='Argument'
        className='input input-bordered'
      ></input>
      <button
        className='btn btn-outline'
        onClick={e => {
          e.preventDefault()
          if (!arg) return alert('Please enter an argument')
          else {
            setArgs([...args, arg])
            setArg('')
          }
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
