<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>select_-- Pilih --        Junior High Schoo_e0e036</name>
   <tag></tag>
   <elementGuidId>00e49e5e-baab-4a42-b5c9-f4eec802023a</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//select[@id='selectEducationDegree--1']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#selectEducationDegree--1</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#selectEducationDegree--1</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>select</value>
      <webElementGuid>a3f5a1a1-be6f-4a99-be8c-8c45c78d53ad</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-collapse</name>
      <type>Main</type>
      <value>#collapseEducation--1</value>
      <webElementGuid>63fcce41-4b51-4a2e-964e-48ce72ec555e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>selectEducationDegree--1</value>
      <webElementGuid>8020716b-58b7-4a37-baed-26d84fdecf20</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>custom-select required</value>
      <webElementGuid>a2e90d10-01f7-4425-9757-16fb4a9e84bb</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onchange</name>
      <type>Main</type>
      <value>
                if($(this).val() === ''){
                    $('#txtEducationDegree--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#frame').contents().find('#txtEducationDegreeFrame--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#txtEducationDegreeFrame--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                } else {
                    $('#txtEducationDegree--1').text($(this).find('option:selected').text()+' ').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#frame').contents().find('#txtEducationDegreeFrame--1').text($(this).find('option:selected').text()+' ').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#txtEducationDegreeFrame--1').text($(this).find('option:selected').text()+' ').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                }
                isEducationValueBlank(); validateRequire($(this), 'collapseEducation'); checkDegree($(this)); autoSave();</value>
      <webElementGuid>70e4b5cd-c972-437b-a8fe-f546cec8a4f8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                -- Pilih --
        Junior High School
        Senior High School
        Associate Degree (D1)
        Associate Degree (D2)
      Associate Degree (D3)
      Bachelor's Degree (D4)
      Bachelor's Degree (S1)
      Master Degree
      Doctoral Degree
              </value>
      <webElementGuid>e72695be-b4bf-4073-abb8-a4a69187f2be</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;selectEducationDegree--1&quot;)</value>
      <webElementGuid>0f523e2c-52b0-4dc7-ae74-b480c0ece1e0</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//select[@id='selectEducationDegree--1']</value>
      <webElementGuid>e97c22a3-bc10-4b6a-af00-b148501a7689</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='collapseEducation--1']/div[2]/div[2]/select</value>
      <webElementGuid>a8f4cade-ff8b-4425-9fa6-88913a0ae624</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Jenjang'])[1]/following::select[1]</value>
      <webElementGuid>916e00b5-8963-4f47-9445-9fe8be1f0d41</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Universitas / Sekolah'])[1]/following::select[1]</value>
      <webElementGuid>5596825d-b69b-4811-b7fd-80eb27f39d94</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Jurusan'])[1]/preceding::select[1]</value>
      <webElementGuid>6548520e-a6c4-488b-af19-d20d299a9eb8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='IPK'])[1]/preceding::select[1]</value>
      <webElementGuid>cafeefd6-59fe-4f3c-9565-29b1d6269477</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div[2]/select</value>
      <webElementGuid>cd15516a-4759-4f1c-99fa-60738d889ee0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//select[@id = 'selectEducationDegree--1' and (text() = concat(&quot;
                -- Pilih --
        Junior High School
        Senior High School
        Associate Degree (D1)
        Associate Degree (D2)
      Associate Degree (D3)
      Bachelor&quot; , &quot;'&quot; , &quot;s Degree (D4)
      Bachelor&quot; , &quot;'&quot; , &quot;s Degree (S1)
      Master Degree
      Doctoral Degree
              &quot;) or . = concat(&quot;
                -- Pilih --
        Junior High School
        Senior High School
        Associate Degree (D1)
        Associate Degree (D2)
      Associate Degree (D3)
      Bachelor&quot; , &quot;'&quot; , &quot;s Degree (D4)
      Bachelor&quot; , &quot;'&quot; , &quot;s Degree (S1)
      Master Degree
      Doctoral Degree
              &quot;))]</value>
      <webElementGuid>f7b8ba0d-c314-416e-b76b-ce996ad1a868</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
