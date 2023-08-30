'use client'

import React from "react";
import {invoke} from "@tauri-apps/api";

export default class AsyncCommand extends React.Component {

    props!: {
        bound: number
    }

    state: {
        msg: string
    }

    constructor(props: any) {
        super(props);

        this.props = {
            bound: props.bound
        };

        this.state = {
            msg: ''
        };
    }

    async componentDidMount() {
        let msg = await invoke('async_command', {value: 'Solution of 1+..+' + this.props.bound, bound: this.props.bound})
            .then((res) => {
                return res;
            })
            .catch((err) => {
                return err;
            })

        await invoke('get_window_command');

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
