import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function ThumbsUpIcon(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 14 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M2.60087 12.1834V12.8241C2.60087 13.2662 2.2425 13.6246 1.80043 13.6246V13.6246C1.35837 13.6246 1 13.2662 1 12.8241V6.00862C1 5.56656 1.35837 5.20819 1.80043 5.20819V5.20819C2.2425 5.20819 2.60087 5.56656 2.60087 6.00862V6.7519M2.60087 12.1834L3.11614 12.7045C3.51836 13.1114 3.71948 13.3148 3.97139 13.4603C4.05154 13.5066 4.14511 13.5526 4.23071 13.5878C4.49977 13.6984 4.76901 13.7317 5.3075 13.7981C6.04574 13.8892 6.95061 13.9848 7.58799 13.9982C8.06675 14.0082 8.62775 13.9757 9.17321 13.9262C10.103 13.8418 10.5679 13.7996 11.004 13.5283C11.1372 13.4453 11.2868 13.3261 11.3973 13.2146C11.7589 12.8499 11.9178 12.3624 12.2356 11.3873L13.1737 8.50878C13.5945 7.21759 12.6952 5.87322 11.3411 5.76935L9.97965 5.66491C9.46342 5.62531 9.09569 5.14654 9.19058 4.63756V4.63756C9.45959 3.62818 9.45892 2.76007 9.18856 1.84912C9.02936 1.31272 8.54202 1 8.02811 1C7.66828 1 7.32598 1.16537 7.18071 1.52677C6.89614 2.23473 6.41573 3.69051 5.71547 4.45923C4.67473 5.60171 3.91029 6.26576 2.60087 6.7519M2.60087 12.1834V6.7519"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}
