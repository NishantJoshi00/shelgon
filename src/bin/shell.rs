fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let app = sheller::renderer::App::<sheller::sample::echosh::Executor>::new(rt)?;

    app.execute()
}
