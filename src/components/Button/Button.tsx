import styles from './Button.module.scss'

export const enum ButtonType {
    PRIMARY,
    SECONDARY
}

interface ButtonProps {
    type?: ButtonType
    link?: string
    onClick: (e: React.MouseEvent<HTMLElement>) => void
    children: React.ReactNode
}

export const Button = (props: ButtonProps) => {
    const buttonClass = props.type === ButtonType.SECONDARY ? styles.buttonSecondary : styles.buttonPrimary

    if (props.link) {
        return (
            <a className={`${styles.button} ${buttonClass}`} target='_blank' href={props.link}>
                {props.children}
            </a>
        )
    }

    return (
        <button className={`${styles.button} ${buttonClass}`} onClick={props.onClick}>
            {props.children}
        </button>
    )
}
