'use client'

import React from "react";
import {invoke} from "@tauri-apps/api";


interface ErrorHandleProps {
}

interface ErrorHandleState {
    msg: string
}

export default class ErrorHandle extends React.Component<ErrorHandleProps, ErrorHandleState> {

    constructor(props: any) {
        super(props);

        this.state = {
            msg: ''
        };
    }

    async componentDidMount() {

        let msg = await invoke('error_handle_command')
            .then((message) => {
                return message as string;
            })
            .catch((error) => {
                return error;
            })

        this.setState({msg: msg})
    }

    render() {
        return (
            <h1>
                {this.state.msg}
            </h1>
        )
    }
}
