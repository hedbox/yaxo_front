<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>New Request Copy</name>
   <tag></tag>
   <elementGuidId>9f590c58-5178-4bcb-9773-46f94ff2597d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n        \&quot;id\&quot;: \&quot;a62cf6d8-d745-45f2-aeae-6d2c8477c7e5\&quot;,\r\n        \&quot;IHM_ROLECODE\&quot;: {\r\n            \&quot;value\&quot;: \&quot;CS\&quot;\r\n        },\r\n        \&quot;IHM_NOM\&quot;: {\r\n            \&quot;value\&quot;: \&quot;AMMARI\&quot;\r\n        },\r\n        \&quot;IHM_prenom\&quot;: {\r\n            \&quot;value\&quot;: \&quot;Tess\&quot;\r\n        },\r\n        \&quot;statut_invitation\&quot;: 0,\r\n        \&quot;mailsent\&quot;: false,\r\n        \&quot;estSignataire\&quot;: false,\r\n        \&quot;YZ_NAME\&quot;: {\r\n            \&quot;value\&quot;: \&quot;AMMARI Tess\&quot;\r\n        },\r\n        \&quot;YZ_JOB_TITLE\&quot;: {\r\n            \&quot;value\&quot;: \&quot;\&quot;\r\n        },\r\n        \&quot;YZ_PHONE\&quot;: {\r\n            \&quot;value\&quot;: \&quot;\&quot;\r\n        },\r\n        \&quot;YZ_EMAIL\&quot;: {\r\n            \&quot;value\&quot;: \&quot;eddy.ammari@gmail.com\&quot;\r\n        }\r\n    }&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${URL_API_INTERNE}/v1/Contacts/updatesignataire?idContact=a62cf6d8-d745-45f2-aeae-6d2c8477c7e5&amp;idFournisseur=1b794421-2de8-4707-893e-032f96807bea&amp;estSignataire=true</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.URL_API_INTERNE</defaultValue>
      <description></description>
      <id>6c01166a-5f86-4943-9b77-6c245be0f339</id>
      <masked>false</masked>
      <name>URL_API_INTERNE</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
