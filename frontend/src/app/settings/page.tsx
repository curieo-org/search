import LogoutSection from '@/components/settings/log-out-section'
import SettingsContent from '@/components/settings/settings-content'
import SettingsNavmenu from '@/components/settings/settings-navmenu'

export default function Settings() {
  return (
    <div className="w-full mt-10 flex justify-center items-center">
      <div className="flex gap-x-12">
        <SettingsNavmenu className="h-36" />
        <div className="flex flex-col bg-white/2 rounded-2.5xl p-5">
          <SettingsContent />
          <LogoutSection className="mt-8" />
        </div>
      </div>
    </div>
  )
}
