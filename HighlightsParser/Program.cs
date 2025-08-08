class Program
{
    static async Task Main()
    {
        ImportEnv.Load(".env");

        string? url = ImportEnv.GetValue("WTT");
        if (url != null)
        {
            await EntryParseWrapper.Run(url);
        }
        else
        {
            Console.WriteLine("Url is null");
        }
    }
}
