import { createFileRoute, redirect } from '@tanstack/react-router'
import { auth } from '../lib/auth'
import { profile } from '../lib/api'

export const Route = createFileRoute('/_auth')({
  // Before loading, authenticate the user via our auth context
  // This will also happen during prefetching (e.g. hovering over links, etc)
  beforeLoad: async ({ context, location }) => {
    // If the user is logged out, redirect them to the login page

    const me = await profile()
    if (!me.data.id) {
      throw redirect({
        to: '/login',
        search: {
          // Use the current location to power a redirect after login
          // (Do not use `router.state.resolvedLocation` as it can
          // potentially lag behind the actual current location)
          redirect: location.href,
        },
      })
    }

    // Otherwise, return the user in context
    return {
      username: auth.username,
    }
  },
})
