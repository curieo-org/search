import { copyResetTime } from '@/constants/config'
import { copyToClipboard } from '@/helpers/browser'
import { useSearchReactionMutation } from '@/queries/search/search-reaction-query'
import { useQueryClient } from '@tanstack/react-query'
import { HTMLAttributes, useState } from 'react'
import { MdDone } from 'react-icons/md'
import { twMerge } from 'tailwind-merge'
import CopyIcon from '../icons/copy'
import RefreshIcon from '../icons/refresh'
import ThumbsDownIcon from '../icons/thumbs-down'
import ThumbsUpIcon from '../icons/thumbs-up'
import { IconButton } from '../lib/button'

type SearchActionsProps = HTMLAttributes<HTMLDivElement> & {
  searchHistoryId: string
  reaction: boolean | null
  response: string
}

export default function SearchActions(props: SearchActionsProps) {
  const queryClient = useQueryClient()
  const [isCopied, setIsCopied] = useState(false)
  const { isPending: isReacting, mutate: saveReaction } = useSearchReactionMutation()

  const handleReaction = async (reaction: boolean) => {
    saveReaction({ search_history_id: props.searchHistoryId, reaction })
  }

  const handleCopyResponse = () => {
    if (!isCopied) {
      copyToClipboard(props.response)

      setIsCopied(true)
      setTimeout(() => {
        setIsCopied(false)
      }, copyResetTime)
    }
  }

  const handleReload = () => {
    queryClient.invalidateQueries({ queryKey: ['search-by-id', props.searchHistoryId] })
  }

  const buttonClassname = 'w-7 h-7 rounded-full hover:bg-white/10'
  const iconClassName = { deafult: 'stroke-white stroke-opacity-70', pressed: 'stroke-custom-purple-300' }

  return (
    <div className={twMerge('w-full flex gap-x-2.5', props.className)}>
      <IconButton
        className={buttonClassname}
        icon={
          <ThumbsUpIcon size={14} className={props.reaction === true ? iconClassName.pressed : iconClassName.deafult} />
        }
        onClick={e => handleReaction(true)}
      />
      <IconButton
        className={buttonClassname}
        icon={
          <ThumbsDownIcon
            size={14}
            className={props.reaction === false ? iconClassName.pressed : iconClassName.deafult}
          />
        }
        onClick={e => handleReaction(false)}
      />
      <IconButton
        className={buttonClassname}
        icon={
          <>
            {isCopied ? (
              <MdDone size={14} className="text-white text-opacity-70" />
            ) : (
              <CopyIcon size={14} className={iconClassName.deafult} />
            )}
          </>
        }
        onClick={handleCopyResponse}
      />
      <IconButton
        className={buttonClassname}
        icon={<RefreshIcon size={14} className={iconClassName.deafult} onClick={handleReload} />}
      />
    </div>
  )
}
