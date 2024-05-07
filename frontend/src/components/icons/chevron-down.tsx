import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function ChevronDownIcon(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 16 10" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M8 9.25C7.8125 9.25 7.65625 9.1875 7.5 9.0625L0.3125 2C0.03125 1.71875 0.03125 1.28125 0.3125 1C0.59375 0.71875 1.03125 0.71875 1.3125 1L8 7.53125L14.6875 0.9375C14.9688 0.65625 15.4062 0.65625 15.6875 0.9375C15.9688 1.21875 15.9688 1.65625 15.6875 1.9375L8.5 9C8.34375 9.15625 8.1875 9.25 8 9.25Z"
        fill="currentColor"
      />
    </svg>
  )
}
