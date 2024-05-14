import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function RefreshIcon(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M7.5 1C11.0899 1 14 3.91015 14 7.5C14 9.9661 12.6266 12.1114 10.603 13.2129M13.1565 13.5562H10.603V13.2129M10.603 13.2129V11.0028M7.5 14C3.91015 14 1 11.0899 1 7.5C1 5.0339 2.37335 2.88857 4.39697 1.78709M1.84353 1.44378L4.39697 1.44378V1.78709M4.39697 1.78709L4.39697 3.99722"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}
