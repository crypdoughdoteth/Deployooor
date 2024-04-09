import React, { useState }  from "react";




export const CreateAddressesPage = () => {
    const [args, setArgs] = useState<any[]>([]);
    const [address, setAddress] = useState<string>('');
  return (
    <form>
        <input
        type='text'
        onChange={(e) => setAddress(e.target.value)}
        placeholder="address"
        >
        </input>
      <button onSubmit={() => setArgs([...args, ''])}>Add Constructor Arguments</button>
      {args.map((i)=>(
        <h2>{i}</h2>
      ))}
    </form>
  );
};