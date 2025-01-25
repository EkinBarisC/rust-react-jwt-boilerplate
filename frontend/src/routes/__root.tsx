import {
  createRootRouteWithContext,
  Link,
  Outlet,
} from "@tanstack/react-router";
import { TAuth } from "../lib/auth";
import { QueryClient } from "@tanstack/react-query";

export const Route = createRootRouteWithContext<{
  auth: TAuth;
  queryClient: QueryClient;
}>()({
  component: Root,
});

function Root() {
  return (
    <>
      <div className="p-2 flex gap-2">
        <Link to="/" className="[&.active]:font-bold">
          Home
        </Link>
        <Link to="/register" className="[&.active]:font-bold">
          Register
        </Link>
        <Link to="/login" className="[&.active]:font-bold">
          Login
        </Link>
        <Link to="/profile" className="[&.active]:font-bold">
          Profile is Protected by Auth
        </Link>
      </div>
      <hr />
      <Outlet />
    </>
  );
}
