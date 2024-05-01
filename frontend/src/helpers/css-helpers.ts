export function getHTMLTextWidth(text: string, font: string = 'normal 16px Times New Roman'): number {
  if (document === undefined) {
    return 1
  }

  const canvas = document.createElement('canvas')
  const context = canvas.getContext('2d')
  context!.font = font
  const metrics = context!.measureText(text)

  return metrics.width
}
