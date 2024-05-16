import { loginPagePath } from '@/constants/route'
import axios from 'axios'

const AxiosClient = axios.create({
  baseURL: '/backend-api',
  withCredentials: true,
})

const onRequest = async (request: any) => {
  return request
}

const onRequestError = (error: any) => {
  return Promise.reject(error)
}

const onResponse = (response: any) => {
  return response
}

const onResponseError = async (error: any) => {
  if (error.response.status === 405 && window.location.pathname !== loginPagePath) {
    window.location.pathname = loginPagePath
  }
  return Promise.reject(error)
}

AxiosClient.interceptors.request.use(onRequest, onRequestError)
AxiosClient.interceptors.response.use(onResponse, onResponseError)

export { AxiosClient }
