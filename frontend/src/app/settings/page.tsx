import LogoutSection from '@/components/settings/log-out-section'
import SettingsNavmenu from '@/components/settings/settings-navmenu'

export default function Settings() {
  return (
    <div className="w-full mt-10 flex justify-center items-center">
      <div className="flex gap-x-10">
        <SettingsNavmenu />
        <LogoutSection />
      </div>
    </div>
  )
}
