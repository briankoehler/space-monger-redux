import { dialog, invoke } from '@tauri-apps/api'
import { WebviewWindow } from '@tauri-apps/api/window'
import { Button, ButtonType } from '../../components/Button/Button'
import { isTree } from '../../models/tree'
import styles from './App.module.scss'

export const App = () => {
    const handleClick = async (e: React.MouseEvent<HTMLElement>) => {
        const path = await dialog.open({ directory: true })
        const tree = await invoke('build_tree_command', { path: path })
        if (!isTree(tree)) dialog.message('Error', 'Invalid tree')
    }

    const openSettings = (e: React.MouseEvent<HTMLElement>) => {
        const settingsWindow = new WebviewWindow('Settings', { url: '/settings' })
    }

    return (
        <>
            <main className={styles.titleContainer}>
                <h1>Space Monger Redux</h1>
                
                <div>
                    <Button type={ButtonType.PRIMARY} onClick={handleClick}>Open</Button>
                    <Button type={ButtonType.SECONDARY} onClick={openSettings}>Settings</Button>
                    <Button type={ButtonType.SECONDARY} onClick={() => {}} link='https://github.com/briankoehler/space-monger-redux'>
                        GitHub
                    </Button>
                </div>
            </main>

            <p className={styles.signature}>Developed by Brian Koehler</p>
        </>
    )
}
