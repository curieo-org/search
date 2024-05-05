import { useEffect, useState } from 'react'
import { ZodSchema } from 'zod'

export function useInputValidation(input: string, schema: ZodSchema) {
  const [errorMessage, setErrorMessage] = useState('')
  const [isError, setIsError] = useState(false)

  useEffect(() => {
    const result = schema.safeParse(input)

    setIsError(!result.success)

    if (!result.success) {
      setErrorMessage(result.error.errors[0]?.message)
    } else {
      setErrorMessage('')
    }
  }, [input])

  return { isError, errorMessage }
}
