import { useQuery } from '@tanstack/react-query'
import { createFileRoute, useRouter } from '@tanstack/react-router'
import { acceptInvite, getAllInvites, inviteFriend, profile } from '../lib/api'

export const Route = createFileRoute('/_auth/profile')({
  component: ProfileComponent,
})

function ProfileComponent() {
  const { auth } = Route.useRouteContext()
  const profileQuery = useQuery({ queryKey: ['me'], queryFn: profile })
  const invites = useQuery({ queryKey: ['invites'], queryFn: () => getAllInvites(profileQuery.data?.data.id) })


  const router = useRouter()
  const handleLogout = () => {
    auth.logout()
    router.invalidate()
  }

  const handleInvite = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)
    const friendIdentifier = formData.get('friendIdentifier') as string
    const id = profileQuery?.data?.data?.id
    await inviteFriend(id, friendIdentifier)
  }


  return (
    <div className="p-2 space-y-2">
      <div>
        Username:<strong>{profileQuery?.data?.data?.username}</strong>
        <br />
        Id:<strong>{profileQuery?.data?.data?.id}</strong>
        <br />
        <button onClick={handleLogout}>Log out</button>
        <div>
          GET ME QUERY:
          {JSON.stringify(profileQuery)}
        </div>

        <form onSubmit={handleInvite}>
          <input type="text" placeholder="Friend Identifier" name="friendIdentifier" />
          <button type="submit">Invite</button>
        </form>

        <h1>invites</h1>
        <ul>
          {invites?.data?.data?.map((invite) => (
            < li key={invite.id} >
              {JSON.stringify(invite)} invited you
              <br />
              <button onClick={async () => {
                await acceptInvite(invite.id, profileQuery?.data?.data?.id)
              }}>Accept</button>
            </li>
          ))}
        </ul>

      </div>
    </div >
  )
}
