

declare function InvokeType<T = any>(
  cmd: keyof Commands,
  args?: Commands[T]
): Promise<T>;

declare global{
    interface Window{
        invoke: InvokeType
    }
}
export {}