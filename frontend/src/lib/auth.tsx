import { login, logout, profile } from "./api";

export const auth: TAuth = {
  username: undefined,
  id: undefined,
  login: async (email: string, password: string) => {
    const res = await login(email, password);
    auth.username = res.data.username;
    auth.id = res.data.id;
  },
  logout: async () => {
    auth.username = undefined;
    await logout();
  },
  isAuthenticated: async () => {
    const res = await profile();
    auth.username = res.data.username;
    auth.id = res.data.id;
    return true;
  },
};

export type TAuth = {
  id?: string;
  username?: string;
  login: (email: string, password: string) => void;
  logout: () => void;
  isAuthenticated: () => Promise<boolean>;
};
