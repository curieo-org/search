export function encodeAsUrlSearchParams(record: Record<string, any> = {}): string {
  const params = new URLSearchParams()
  Object.entries(record).forEach(entry => {
    const [key, value] = entry
    if (Array.isArray(value)) {
      value.forEach(v => params.append(key, v))
    } else {
      params.append(key, value)
    }
  })
  return params.toString()
}

export function formToUrlParams(formData: FormData): URLSearchParams {
  return new URLSearchParams(
    Array.from(formData, ([key, value]) => [key, typeof value === 'string' ? value : value.name])
  )
}
