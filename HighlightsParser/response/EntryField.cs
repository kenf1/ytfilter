using System.Xml.Linq;

class EntryField
{
    public static XElement? GetFirstEntry(XDocument doc, XNamespace atomNamespace)
    {
        return doc.Root?.Element(atomNamespace + "entry");
    }

    public static IEnumerable<XElement> GetAllEntries(XDocument doc, XNamespace atomNamespace)
    {
        return doc.Root?.Elements(atomNamespace + "entry") ?? Enumerable.Empty<XElement>();
    }

    public static bool CheckEntryEmpty(XElement? entry)
    {
        if (entry != null)
        {
            return true;
        }
        else
        {
            Console.WriteLine("No entries found in XML.");
            return false;
        }
    }
}