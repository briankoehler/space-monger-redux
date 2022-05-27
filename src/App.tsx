import { dialog } from '@tauri-apps/api'
import styles from './App.module.scss'
import { Button, ButtonType } from './Button/Button'

const App = () => {
    const handleClick = async (e: React.MouseEvent<HTMLElement>) => {
        await dialog.open({ directory: true })
    }

    return (
        <>
            <main className={styles.titleContainer}>
                <h1>Space Monger Redux</h1>
                
                <div>
                    <Button type={ButtonType.PRIMARY} onClick={handleClick}>Open</Button>
                    <Button type={ButtonType.SECONDARY} onClick={handleClick}>Settings</Button>
                </div>
            </main>

            <p className={styles.signature}>Developed by Brian Koehler</p>
        </>
    )
}

export default App
