
    import axios, { AxiosError, AxiosResponse } from 'axios';

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080/api/v1',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },
});


apiClient.interceptors.response.use(
  (response: AxiosResponse) => response.data,
  (error: AxiosError) => {
    const errorMessage = error.response?.data?.message || 'Failed to connect to server';
    console.error('API Error:', errorMessage);
    return Promise.reject(new Error(errorMessage));
  }
);

export default apiClient;
    