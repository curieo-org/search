import { H1 } from '@/components/lib/typography'
import AccountDetails from '@/components/settings/account-details'
import LogoutSection from '@/components/settings/log-out-section'
import ProfileInfo from '@/components/settings/profile-info'

export default function Settings() {
  return (
    <div className="w-full mt-10 flex justify-center items-center">
      <div className="w-[480px] xl:w-[600px] mx-auto flex flex-col items-center">
        <H1 className="mb-8 text-3xl font-semibold">Settings</H1>
        <ProfileInfo className="mb-8" />
        <AccountDetails className="mb-2" />
        <LogoutSection />
      </div>
    </div>
  )
}
