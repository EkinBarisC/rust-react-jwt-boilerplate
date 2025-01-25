import { createFileRoute } from '@tanstack/react-router'
import { register } from '../lib/api';

export const Route = createFileRoute('/register')({
  component: RouteComponent,
})

function RouteComponent() {
  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);
    const username = formData.get('username') as string;
    const email = formData.get('email') as string;
    const password = formData.get('password') as string;
    console.log({ username, email, password });
    await register({ username, email, password });
  }

  return (
    <form onSubmit={handleSubmit}>
      <input type="text" name="username" placeholder="username" />
      <input type="email" name="email" placeholder="email" />
      <input type="password" name="password" placeholder="password" />
      <button type="submit">Register</button>
    </form>
  )
}
