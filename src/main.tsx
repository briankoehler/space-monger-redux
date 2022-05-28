import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { App } from './routes/App/App'
import { Settings } from './routes/Settings/Settings'
import './styles/index.scss'

interface RouteData {
    path: string
    element: JSX.Element
}

const routes: RouteData[] = [
    {
        path: '/',
        element: <App />,
    },
    {
        path: '/settings',
        element: <Settings />,
    },
]

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <BrowserRouter>
            <Routes>
                {routes.map(data => (
                    <Route key={data.path} {...data} />
                ))}
            </Routes>
        </BrowserRouter>
    </React.StrictMode>
)
