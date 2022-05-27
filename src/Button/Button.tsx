import styles from './Button.module.scss'

export const enum ButtonType {
    PRIMARY,
    SECONDARY
}

interface ButtonProps {
    type?: ButtonType
    onClick: (e: React.MouseEvent<HTMLElement>) => void
    children: React.ReactNode
}

export const Button = (props: ButtonProps) => {
    const buttonClass = props.type === ButtonType.SECONDARY ? styles.buttonSecondary : styles.buttonPrimary

    return (
        <button className={`${styles.button} ${buttonClass}`} onClick={props.onClick}>
            {props.children}
        </button>
    )
}
