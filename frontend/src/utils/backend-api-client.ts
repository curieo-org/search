import { signinPagePath } from '@/constants/route'
import axios from 'axios'

const BackendAPIClient = axios.create({
  baseURL: `${process.env.AUTH_URL}/backend-api/`,
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
  if (error.response && 'status' in Object.keys(error.response)) {
    if (error.response.status === 405 && window.location.pathname !== signinPagePath) {
      window.location.pathname = signinPagePath
    }
  }
  return Promise.reject(error)
}

BackendAPIClient.interceptors.request.use(onRequest, onRequestError)
BackendAPIClient.interceptors.response.use(onResponse, onResponseError)

export { BackendAPIClient }
