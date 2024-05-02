import axios from 'axios'

const AxiosClient = axios.create({
  baseURL: process.env.API_BASE_URL,
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
  if (error.response.status === 405) {
    window.location.pathname = '/authentication'
  }
  return Promise.reject(error)
}

AxiosClient.interceptors.request.use(onRequest, onRequestError)
AxiosClient.interceptors.response.use(onResponse, onResponseError)

export { AxiosClient }
