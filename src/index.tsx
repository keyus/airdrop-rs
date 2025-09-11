import { useState } from "react";
import { useMount } from "ahooks";
import { createRoot } from "react-dom/client";
import { ConfigContext, initConfigValue } from '@/rootContext'
import { ConfigProvider, App as AntdApp, message } from 'antd'
import { createHashRouter, RouterProvider, } from 'react-router'
import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import App from "./App";
import zhCN from 'antd/locale/zh_CN';
import Chrome from './pages/chrome'
import ChromeApp from './pages/chrome_app'
import Setting from './pages/setting'
import '@ant-design/v5-patch-for-react-19';

window.invoke = function <T = any>(cmd: keyof Commands, args?: Record<string, any>): Promise<T> {
    return tauriInvoke(cmd, args)
};
message.config({
    maxCount: 1,
});

function Main() {
    const [config, setConfig] = useState<AppConfig>(initConfigValue);
    const updateConfig = async (newConfig: AppConfig) => {
        await window.invoke('set_config', { config: newConfig });
        message.success('配置保存成功');
    }
    const init = async () => {
        const res = await window.invoke<ConfigPromise>('get_config');
        setConfig(res.data);
    }
    useMount(() => {
        init();
    })
    return (
        <AntdApp className="root-app">
            <ConfigProvider
                locale={zhCN}
                theme={{
                    components: {
                        Table: {
                            headerBg: '#fff',
                            headerColor: '#bcbec5',
                            cellFontSize: 13,
                            footerBg: '#fff',
                            rowHoverBg: '#e9edfd',
                        }
                    }
                }}
            >
                <ConfigContext value={{
                    config,
                    refreshConfig: init,
                    updateConfig,
                }}>
                    <RouterProvider
                        router={createHashRouter([
                            {
                                path: '/',
                                element: <App />,
                                children: [
                                    {
                                        index: true,
                                        element: <Chrome />,
                                        handle: {
                                            title: '环境管理'
                                        }
                                    },
                                    {
                                        path: 'chrome_app',
                                        element: <ChromeApp />,
                                        handle: {
                                            title: '浏览器应用'
                                        }
                                    },
                                    {
                                        path: 'setting',
                                        element: <Setting />,
                                        handle: {
                                            title: 'app设置'
                                        }
                                    }
                                ]
                            },
                        ])}
                    />
                </ConfigContext>
            </ConfigProvider>
        </AntdApp>
    )
}

createRoot(document.getElementById("root") as HTMLElement).render(<Main />);

