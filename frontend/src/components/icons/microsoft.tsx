import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function MicrosoftIcon(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
      <g clip-path="url(#clip0_141_337)">
        <path
          d="M0.3125 9.5625C3.40625 9.5625 6.46875 9.5625 9.5625 9.5625C9.5625 6.46875 9.5625 3.40625 9.5625 0.3125H0.3125V9.5625Z"
          fill="currentColor"
        />
        <path
          d="M19.687 9.5625C19.687 6.46875 19.687 3.40625 19.687 0.3125H10.437C10.437 3.40625 10.437 6.46875 10.437 9.5625C13.5308 9.5625 16.5933 9.5625 19.687 9.5625Z"
          fill="currentColor"
        />
        <path
          d="M0.3125 19.6876H9.5625C9.5625 16.5939 9.5625 13.5314 9.5625 10.4376C6.46875 10.4376 3.40625 10.4376 0.3125 10.4376V19.6876Z"
          fill="currentColor"
        />
        <path
          d="M10.437 19.6876H19.687C19.687 16.5939 19.687 13.5314 19.687 10.4376C16.5933 10.4376 13.5308 10.4376 10.437 10.4376C10.437 13.5314 10.437 16.5939 10.437 19.6876Z"
          fill="currentColor"
        />
      </g>
      <defs>
        <clipPath id="clip0_141_337">
          <rect width="20" height="20" fill="white" />
        </clipPath>
      </defs>
    </svg>
  )
}
