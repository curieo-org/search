import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function ShiftRight(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 23 35" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M21.5 5C22.0455 5 22.5 5.41391 22.5 5.9106V29.0894C22.5 29.5861 22.0455 30 21.5 30C20.9545 30 20.5 29.5861 20.5 29.0894V5.9106C20.5 5.41391 20.9545 5 21.5 5Z"
        fill="currentColor"
      />
      <path
        d="M1.44029 16.5702H14.2197L11.2706 13.6285C10.8859 13.2502 10.8859 12.6619 11.2706 12.2837C11.6552 11.9054 12.2536 11.9054 12.6383 12.2837L17.2115 16.8223C17.5962 17.2006 17.5962 17.7889 17.2115 18.1671L12.681 22.7058C12.5101 22.8739 12.2536 23 11.9972 23C11.7407 23 11.527 22.916 11.3133 22.7479C10.9287 22.3696 10.9287 21.7813 11.3133 21.4031L14.2197 18.4613H1.44029C0.927402 18.4613 0.5 18.0411 0.5 17.5368C0.5 17.0325 0.884663 16.5702 1.44029 16.5702Z"
        fill="currentColor"
      />
    </svg>
  )
}
