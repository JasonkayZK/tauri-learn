import styles from './page.module.css'
import Greet from "@/app/components/Greet";
import React from "react";
import AsyncCommand from "@/app/components/AsyncCommand";
import ErrorHandle from "@/app/components/ErrorHandle";

export default function Home() {
    return (
        <main className={styles.main}>
            <Greet name='Next.js'/>

            <ErrorHandle></ErrorHandle>

            <AsyncCommand bound={100}></AsyncCommand>
        </main>
    )
}
