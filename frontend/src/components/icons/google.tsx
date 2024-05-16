import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function GoogleIcon(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 19 20" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M18.7193 9.12503H9.7505V11.7813H16.188C15.8443 15.4688 12.8443 17.0625 9.938 17.0625C6.2505 17.0625 2.96925 14.1875 2.96925 10.0313C2.96925 6.03128 6.09425 3.00003 9.938 3.00003C12.8755 3.00003 14.6568 4.90628 14.6568 4.90628L16.4693 3.00003C16.4693 3.00003 14.0318 0.343784 9.813 0.343784C4.2505 0.312534 0.000488281 4.96878 0.000488281 10C0.000488281 14.875 4.0005 19.6875 9.90675 19.6875C15.1255 19.6875 18.8755 16.1563 18.8755 10.875C18.9068 9.78128 18.7193 9.12503 18.7193 9.12503Z"
        fill="currentColor"
      />
    </svg>
  )
}
