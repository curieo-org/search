import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function ClockIcon(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <g clipPath="url(#clip0_30_2767)">
        <path
          d="M11.65 7.92505H8.37502V4.22505C8.37502 3.92505 8.12502 3.67505 7.82502 3.67505C7.52502 3.67505 7.27502 3.92505 7.27502 4.22505V8.50005C7.27502 8.80005 7.52502 9.05005 7.82502 9.05005H11.65C11.95 9.05005 12.2 8.80005 12.2 8.50005C12.2 8.20005 11.975 7.92505 11.65 7.92505Z"
          fill="currentColor"
        />
        <path
          d="M8.00001 0.350098C3.82501 0.350098 0.450012 3.7751 0.450012 8.0001C0.450012 12.2251 3.82501 15.6501 8.00001 15.6501C12.175 15.6501 15.55 12.2251 15.55 8.0001C15.55 3.7751 12.175 0.350098 8.00001 0.350098ZM8.00001 14.5251C4.45001 14.5251 1.55001 11.6001 1.55001 8.0001C1.55001 4.4001 4.45001 1.4751 8.00001 1.4751C11.55 1.4751 14.45 4.4001 14.45 8.0001C14.45 11.6001 11.55 14.5251 8.00001 14.5251Z"
          fill="currentColor"
        />
      </g>
      <defs>
        <clipPath id="clip0_30_2767">
          <rect width="16" height="16" fill="currentColor" />
        </clipPath>
      </defs>
    </svg>
  )
}
