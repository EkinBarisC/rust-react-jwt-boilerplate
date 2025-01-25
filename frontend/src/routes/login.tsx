import { createFileRoute, useRouter } from '@tanstack/react-router'
import { profile } from '../lib/api'
import { useQuery } from '@tanstack/react-query'

export const Route = createFileRoute('/login')({
  component: RouteComponent,
})

function RouteComponent() {

  const router = useRouter()
  const { auth } = Route.useRouteContext({
    select: ({ auth }) => ({ auth, isAuthenticated: auth.isAuthenticated })
  })

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)
    const username = formData.get('username') as string
    const password = formData.get('password') as string
    await auth.login(username, password)
    router.invalidate()
  }

  const handleLogout = () => {
    auth.logout()
    router.invalidate()
  }
  const me = useQuery({ queryKey: ['me'], queryFn: profile })

  return me?.data?.data?.id ? (
    <div>
      <h1>Already logged in</h1>
      <button onClick={handleLogout}>log out</button>
    </div>
  ) : (
    <div>
      <h1>Login</h1>
      <form onSubmit={handleSubmit}>
        <input type="text" name="username" placeholder="username" />
        <input type="password" name="password" placeholder="password" />
        <button type="submit">Log in</button>
      </form>
    </div>
  )
}
