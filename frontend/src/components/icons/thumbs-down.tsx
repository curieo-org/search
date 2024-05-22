import { getCalculatedSize } from './utils/helpers'
import { IconProps } from './utils/types'

export default function ThumbsDownIcon(props: IconProps) {
  const { size: iconSize, ...rest } = props
  const size = getCalculatedSize(iconSize)

  return (
    <svg {...rest} width={size} height={size} viewBox="0 0 14 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M2.60087 2.81662V2.17586C2.60087 1.7338 2.2425 1.37543 1.80043 1.37543V1.37543C1.35837 1.37543 1 1.7338 1 2.17586V8.99138C1 9.43344 1.35837 9.79181 1.80043 9.79181V9.79181C2.2425 9.79181 2.60087 9.43344 2.60087 8.99138V8.2481M2.60087 2.81662L3.11614 2.29546C3.51836 1.88863 3.71948 1.68521 3.97139 1.53971C4.05154 1.49342 4.14511 1.44742 4.23071 1.41222C4.49977 1.30158 4.76901 1.26835 5.3075 1.20189C6.04574 1.11077 6.95061 1.0152 7.58799 1.00182C8.06675 0.991776 8.62775 1.02429 9.17322 1.07381C10.103 1.15822 10.5679 1.20043 11.004 1.47175C11.1372 1.55467 11.2868 1.67394 11.3973 1.78539C11.7589 2.15007 11.9178 2.63763 12.2356 3.61274L13.1737 6.49122C13.5945 7.78241 12.6952 9.12678 11.3411 9.23065L9.97966 9.33509C9.46342 9.37469 9.09569 9.85346 9.19059 10.3624V10.3624C9.45959 11.3718 9.45892 12.2399 9.18856 13.1509C9.02936 13.6873 8.54202 14 8.02811 14C7.66828 14 7.32598 13.8346 7.18072 13.4732C6.89614 12.7653 6.41573 11.3095 5.71547 10.5408C4.67473 9.39829 3.91029 8.73424 2.60087 8.2481M2.60087 2.81662V8.2481"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}