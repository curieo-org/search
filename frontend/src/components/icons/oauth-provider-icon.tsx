import { ReactNode } from 'react'
import GoogleIcon from '@/components/icons/google'
import { IconProps } from '@/components/icons/utils/types'
import AppleIcon from '@/components/icons/apple'

export type OAuthProviderIconProps = {
  id: string
} & IconProps

export function OAuthProviderIcon(props: OAuthProviderIconProps): ReactNode {
  const { id, ...rest } = props
  switch (id) {
    case 'google':
      return <GoogleIcon {...rest} />
    case 'apple':
      return <AppleIcon {...rest} />
    default:
      return <></>
  }
}
