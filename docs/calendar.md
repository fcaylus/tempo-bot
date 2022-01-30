# 📅 Export your calendar to .ics

The tempo-bot needs an export of your calendar (in `.ics` format) in order to extract your meetings.

> Don't worry, Outlook will create a link that refreshes itself, you only need to do the export once !

## Outlook

1. Go to your Outlook Calendar web interface (for Outlook Cloud, it's at https://outlook.office.com/calendar)
2. Go to `Settings -> View all Outlook settings`
3. In `Calendar -> Shared calendars -> Share a calendar`, share your calendar with your personal email address
   (or any other email). For better parsing of the "Tempo Code", you should select "Can view all details"
4. You will receive an email with a `sharing_metadata.xml` file attached. 
   Open it, and extract the url at `SharingMessage.Invitation.Providers.Provider.ICalUrl` (it should ends with `reachcalendar.ics`)
```xml
<?xml version="1.0"?>
<SharingMessage xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://schemas.microsoft.com/sharing/2008">
  <Invitation>
    <Providers>
      <Provider Type="ms-exchange-publish">
        <BrowseUrl xmlns="http://schemas.microsoft.com/exchange/sharing/2008"></BrowseUrl>
        <ICalUrl xmlns="http://schemas.microsoft.com/exchange/sharing/2008">THE URL OF THE CALENDAR</ICalUrl>
      </Provider>
    </Providers>
  </Invitation>
</SharingMessage>
```
5. Use this url for the `--calendar-ics` parameter
6. Done 🎉

