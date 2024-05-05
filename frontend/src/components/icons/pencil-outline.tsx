import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function PencilOutlineIcon(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M10.9562 0.222863C11.1645 0.0145795 11.5022 0.0145795 11.7104 0.222863L13.8437 2.3562C14.0521 2.56447 14.0521 2.90216 13.8437 3.11044L9.63829 7.31591C9.55722 7.39698 9.46357 7.46444 9.36106 7.51572L5.17183 9.61034C4.96649 9.71305 4.71852 9.67273 4.55619 9.51044C4.39387 9.34811 4.35362 9.10013 4.45629 8.8948L6.55091 4.70555C6.60219 4.603 6.66965 4.50939 6.75072 4.42832L10.9562 0.222863ZM11.3333 1.35423L7.50497 5.18258L6.3975 7.3975L6.66913 7.66913L8.88405 6.56166L12.7124 2.73332L11.3333 1.35423ZM8.66664 1.13332L7.59998 2.19998H3.22667C2.76982 2.19998 2.45926 2.2004 2.21922 2.22C1.9854 2.23911 1.86582 2.27374 1.78241 2.31624C1.5817 2.41851 1.41853 2.58169 1.31625 2.78239C1.27376 2.86581 1.23913 2.98538 1.22003 3.2192C1.20041 3.45925 1.2 3.76981 1.2 4.22665V10.84C1.2 11.2968 1.20041 11.6073 1.22003 11.8475C1.23913 12.0813 1.27376 12.2008 1.31625 12.2843C1.41853 12.485 1.5817 12.6481 1.78241 12.7504C1.86582 12.7929 1.9854 12.8275 2.21922 12.8466C2.45926 12.8662 2.76982 12.8666 3.22667 12.8666H9.84C10.2969 12.8666 10.6074 12.8662 10.8475 12.8466C11.0813 12.8275 11.2009 12.7929 11.2843 12.7504C11.485 12.6481 11.6481 12.485 11.7504 12.2843C11.793 12.2008 11.8275 12.0813 11.8466 11.8475C11.8662 11.6073 11.8667 11.2968 11.8667 10.84V6.46663L12.9333 5.39996V10.84V10.8621C12.9333 11.2913 12.9333 11.6456 12.9098 11.9343C12.8852 12.2341 12.8327 12.5096 12.7008 12.7685C12.4963 13.1699 12.1699 13.4963 11.7685 13.7008C11.5097 13.8327 11.2341 13.8852 10.9343 13.9097C10.6457 13.9333 10.2913 13.9333 9.86208 13.9333H9.84H3.22667H3.20463C2.77532 13.9333 2.42099 13.9333 2.13236 13.9097C1.83257 13.8852 1.557 13.8327 1.29815 13.7008C0.896745 13.4963 0.570377 13.1699 0.365854 12.7685C0.233961 12.5096 0.181396 12.2341 0.156905 11.9343C0.133321 11.6456 0.133321 11.2913 0.133332 10.8621V10.84V4.22665V4.20462C0.133321 3.77533 0.133321 3.42098 0.156905 3.13235C0.181396 2.83256 0.233961 2.55699 0.365854 2.29814C0.570377 1.89672 0.896745 1.57036 1.29815 1.36584C1.557 1.23395 1.83257 1.18138 2.13236 1.15689C2.42099 1.13331 2.77531 1.13331 3.20461 1.13332H3.22667H8.66664Z"
        fill="currentColor"
      />
    </svg>
  )
}