import axios from 'axios';

const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080',
  headers: {
    'Content-Type': 'application/json',
  },
});

api.interceptors.request.use((config) => {
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
  }
  return config;
});

api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      if (typeof window !== 'undefined') {
        localStorage.removeItem('token');
        window.location.href = '/login';
      }
    }
    return Promise.reject(error);
  }
);

export interface Skill {
  id: string;
  name: string;
  description: string | null;
  github_owner: string;
  github_repo: string;
  stars: number;
  forks: number;
  language: string | null;
  tags: string[];
  install_command: string | null;
  price: number;
  marketplace: boolean;
  downloaded_count: number;
  last_synced_at: string | null;
  skill_content?: string | null;
  readme_content?: string | null;
}

export interface PaginatedResponse<T> {
  data: T[];
  page: number;
  limit: number;
  total: number;
  total_pages: number;
}

export const skillsApi = {
  list: async (params?: {
    page?: number;
    limit?: number;
    sort?: string;
    search?: string;
    language?: string;
  }): Promise<PaginatedResponse<Skill>> => {
    const { data } = await api.get('/api/skills', { params });
    return data;
  },

  get: async (id: string): Promise<Skill> => {
    const { data } = await api.get(`/api/skills/${id}`);
    return data;
  },

  download: async (id: string): Promise<{ download_url: string }> => {
    const { data } = await api.get(`/api/skills/${id}/download`);
    return data;
  },
};

export const authApi = {
  register: async (email: string, password: string, name?: string) => {
    const { data } = await api.post('/api/auth/register', { email, password, name });
    if (data.token) {
      localStorage.setItem('token', data.token);
    }
    return data;
  },

  login: async (email: string, password: string) => {
    const { data } = await api.post('/api/auth/login', { email, password });
    if (data.token) {
      localStorage.setItem('token', data.token);
    }
    return data;
  },

  me: async () => {
    const { data } = await api.get('/api/auth/me');
    return data;
  },

  logout: () => {
    localStorage.removeItem('token');
  },
};

export const favoritesApi = {
  list: async () => {
    const { data } = await api.get('/api/favorites');
    return data;
  },

  add: async (skillId: string) => {
    const { data } = await api.post('/api/favorites', { skill_id: skillId });
    return data;
  },

  remove: async (id: string) => {
    const { data } = await api.delete(`/api/favorites/${id}`);
    return data;
  },
};

export default api;
