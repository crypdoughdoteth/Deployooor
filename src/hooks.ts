import { useState } from 'react'

type Status = 'idle' | 'loading' | 'success' | 'error'

export const useStatus = (status: Status) => {
  return useState<Status>(status)
}