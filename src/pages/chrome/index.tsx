
import { useContext, useState, } from 'react'
import { Input, Form, Checkbox, Space, Table, } from 'antd'
import { useInterval, useMount, useUpdate, useUpdateEffect } from 'ahooks'
import { SearchOutlined, } from '@ant-design/icons'
import { ConfigContext } from '@/rootContext'
import Open from './components/Open'
import Close from './components/Close'
import columns from './columns';
import Config from './components/config/Index'
import { genWalletList } from '@/util'
import './style.css';


let originData = []
export default function Chrome() {
    const [form] = Form.useForm();
    const update = useUpdate();
    const { config, } = useContext(ConfigContext);
    const open = Form.useWatch('open', form);
    const search = Form.useWatch('search', form);
    const [data, setData] = useState([])
    const [selectedRowKeys, setSelectedRowKeys] = useState([]);
    const [selectedRows, setSelectedRows] = useState([]);

    const [pagination, setPagination] = useState({
        current: 1,
        pageSize: 7,
    })
    const total = data.length;

    useMount(() => {
        initWalllet()
    })

    useUpdateEffect(() => {
        initWalllet()
    }, [config.wallet])

    const initWalllet = () => {
        if (config.wallet) {
            const wallet = genWalletList(config.wallet)
            originData = wallet;
            setData(wallet)
        }
    }


    // 每3秒检查一次进程是否存在
    useInterval(async () => {
        checkProcess()
    }, 2500)

    useUpdateEffect(() => {
        if (open) {
            setData(data => {
                return data.filter(it => it.openTg || it.openChrome)
            })
        } else {
            setData(originData)
        }
    }, [open]);

    useUpdateEffect(() => {
        if (search) {
            return setData(originData.filter(it => {
                return it.name.toLowerCase().includes(search.toLowerCase())
            }))
        } else {
            setData(originData)
        }
    }, [search])

    // 检查进程是否存在
    const checkProcess = async () => {
        const res = await invoke<ProcessResult>('get_all_open');
        const { chrome, telegram } = res.data;
        data.map((item: any) => {
            if (chrome.find(it => it.name === item.name)) {
                item.openChrome = true
            } else {
                item.openChrome = false
            }
            if (telegram.find(it => it.name === item.name)) {
                item.openTg = true
            } else {
                item.openTg = false
            }
        })
        setData([...data])
    }

    const onChange = (pagination: any) => {
        setPagination(pagination)
    }

    const clearSelected = () => {
        setSelectedRowKeys([])
        setSelectedRows([])
    }

    const openChromeOne = async (name: string) => {
        await window.invoke('open_chrome', { names: [name] })
        window.message.success('chrome,打开成功')
    }
    const closeChromeOne = async (name: string) => {
        await window.invoke('close_chrome', { names: [name] })
        window.message.success('chrome,关闭成功')
    }

    const openTgOne = async (name: string) => {
        await window.invoke('open_telegram', { names: [name] })
        window.message.success('telegram,打开成功')
    }
    const closeTgOne = async (name: string) => {
        await window.invoke('close_telegram', { names: [name] })
        window.message.success('telegram,关闭成功')
    }
    const runShmonad = async (row: any) => {
        const name = row.name;
        row.shmonad = 0;
        update();
        // 运行shmonad
        window.message.open({
            key: 'shmonad',
            type: 'loading',
            content: `${name} shmonad 运行中!`,
            duration: 0,
        });
        try {
            await window?.pywebview.api.testnet.run_shmonad(name);
            window.message.open({
                key: 'shmonad',
                type: 'success',
                content: `${name} shmonad,运行成功!`,
                duration: 5,
            });
            row.shmonad = 1
        } catch {
            row.shmonad = 2
        }
        setData([...data])
    }

    const column = columns({ openChromeOne, openTgOne, closeChromeOne, closeTgOne, runShmonad });
    const x = column.reduce((a, b) => { return a + b.width }, 0)

    return (
        <div style={{ height: '100%' }}>
            <div>
                <Form
                    form={form}
                    layout='inline'
                    initialValues={{
                        group: '',
                    }}
                >
                    <Form.Item
                        name='search'
                    >
                        <Input size="large"
                            max={20}
                            onPressEnter={form.submit}
                            placeholder="搜索名字"
                            allowClear
                            prefix={<SearchOutlined />} />
                    </Form.Item>
                    <Form.Item
                        name='open'
                        valuePropName='checked'
                        className='flex-center'
                    >
                        <Checkbox>已打开(0)</Checkbox>
                    </Form.Item>
                </Form>
            </div>
            <div className='tools'>
                <div>
                    <Space>
                        <Open
                            selectedRows={selectedRows}
                            onOk={clearSelected}
                        />
                        <Close onOk={clearSelected} />
                    </Space>
                </div>
                <div>
                    <Config />
                </div>
            </div>
            <div className='list-table'>
                <Table
                    scroll={{ x }}
                    rowSelection={{
                        fixed: true,
                        selectedRowKeys,
                        onChange: (selectedRowKeys, selectedRows) => {
                            setSelectedRowKeys(selectedRowKeys)
                            setSelectedRows(selectedRows)
                        }
                    }}
                    locale={{ emptyText: '暂无数据' }}
                    rowKey='name'
                    columns={column as any}
                    pagination={{
                        current: pagination.current,
                        pageSize: pagination.pageSize,
                        showSizeChanger: true,
                        pageSizeOptions: [5, 7, 10],
                        total,
                    }}
                    onChange={onChange}
                    dataSource={data}
                />
            </div>
        </div>
    )
}


