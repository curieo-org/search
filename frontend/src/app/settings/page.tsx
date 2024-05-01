import { H1 } from '@/components/lib/typography'
import AccountDetails from '@/components/settings/account-details'
import ProfileInfo from '@/components/settings/profile-info'

export default function Settings() {
  return (
    <div className="w-full mt-10 flex justify-center items-center">
      <div className="w-full flex flex-col gap-y-10 items-center">
        <H1 className="text-3xl font-semibold">Settings</H1>
        <ProfileInfo />
        <AccountDetails />
      </div>
    </div>
  )
}
