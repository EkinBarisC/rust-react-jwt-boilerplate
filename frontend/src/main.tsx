import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"

import { routeTree } from './routeTree.gen'
import { auth } from './lib/auth'

const queryClient = new QueryClient()
const router = createRouter({ routeTree, context: { auth, queryClient } })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider
        router={router}
        context={{ auth }}
      />
    </QueryClientProvider>
  </StrictMode>,
)
