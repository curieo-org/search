export function getCalculatedSize(iconSize: number | string | undefined): string {
  const size = iconSize ?? '1em'

  if (typeof size === 'number') {
    return `${size}px`
  }

  return size
}
