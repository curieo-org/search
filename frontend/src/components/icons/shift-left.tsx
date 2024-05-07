import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function ShiftLeft(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 25 27" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M6.5063 5.95001C6.2063 5.95001 5.9563 6.20001 5.9563 6.50001V20.5C5.9563 20.8 6.2063 21.05 6.5063 21.05C6.8063 21.05 7.0563 20.8 7.0563 20.5V6.50001C7.0563 6.20001 6.8063 5.95001 6.5063 5.95001Z"
        fill="currentColor"
      />
      <path
        d="M19.45 12.9469H11.975L13.7 11.1969C13.925 10.9719 13.925 10.6219 13.7 10.3969C13.475 10.1719 13.125 10.1719 12.9 10.3969L10.225 13.0969C10 13.3219 10 13.6719 10.225 13.8969L12.875 16.5969C12.975 16.6969 13.125 16.7719 13.275 16.7719C13.425 16.7719 13.55 16.7219 13.675 16.6219C13.9 16.3969 13.9 16.0469 13.675 15.8219L11.975 14.0719H19.45C19.75 14.0719 20 13.8219 20 13.5219C20 13.2219 19.775 12.9469 19.45 12.9469Z"
        fill="currentColor"
      />
    </svg>
  )
}
