import axios from "axios";

const API_URL = import.meta.env.VITE_API_URL;

const api = axios.create({
  baseURL: API_URL,
  withCredentials: true,
});

api.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;
    if (error.response.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;
      await api.post(`/api/auth/refresh`, {}, { withCredentials: true });
      return api(originalRequest);
    }
    return Promise.reject(error);
  },
);

export const register = async (data: {
  username: string;
  password: string;
  email: string;
}) => {
  const response = await api.post("/api/auth/register", {
    username: data.username,
    password: data.password,
    email: data.email,
  });
  return response;
};

export const login = async (identifier: string, password: string) => {
  const response = await api.post("/api/auth/login", { identifier, password });
  return response;
};

export const logout = async () => {
  const response = await api.post("/api/auth/logout");
  return response;
};

export const profile = async () => {
  const response = await api.get("/api/auth/profile");
  return response;
};

export default api;
