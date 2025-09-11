/// <reference types="antd/es/message/interface" />

declare module '*.svg' {
  export const ReactComponent: React.FunctionComponent<
    React.SVGProps<SVGSVGElement>
  >;
  const content: string;
  export default content;
}
declare module '*.svg?react' {
  const ReactComponent: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
  export default ReactComponent;
}

declare module '*.png' {
  const content: string;
  export default content;
}

declare module '*.jpg' {
  const content: string;
  export default content;
}


declare interface Window {
  message: MessageInstance
}


interface AppConfig {
  chrome_install_dir?: string;
  chrome_user_data_dir?: string;
  telegram_install_dir?: string;
  use_url?: boolean;
  url?: string[];
  use_proxy?: boolean;
  wallet?: string[];
}

interface ConfigPromise {
  data: AppConfig;
  status: boolean;
}

interface ProcessResult {
  status: boolean;
  data: {
    chrome: {
      pid: number;
      name: string;
    }[];
    telegram: {
      pid: number;
      name: string;
    }[];
  }
}


type Commands = {
  get_config: undefined;
  set_config: undefined;
  get_proxy: undefined;
  set_proxy: undefined;
  clear: undefined;
  my_ip: undefined;
  ip_list: undefined;
  add_auth_ip: undefined;
  remove_auth_ip: undefined;
  auth_ip_list: undefined;

  open_chrome: {names: string[]};
  open_telegram: {names: string[]};
  close_chrome: {names: string[]};
  close_telegram: {names: string[]};
  close_chrome_all: undefined;
  close_telegram_all: undefined;
  add_extensions: { id: string };
  remove_extensions: { id: string };
};


declare function InvokeType<T = any>(
  cmd: keyof Commands,
  args?: Commands[T]
): Promise<T>;

declare global{
    interface window{
        invoke: InvokeType
    }
}